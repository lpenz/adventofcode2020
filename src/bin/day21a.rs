// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::{anyhow, Result};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io::{stdin, BufRead};

// Ingredient, Allergen: interned strings //

#[derive(Clone, Copy)]
pub struct IngredientSymbol;
impl string_intern::Validator for IngredientSymbol {
    type Err = ::std::string::ParseError;
    fn validate_symbol(_: &str) -> Result<(), Self::Err> {
        Ok(())
    }
}
type Ingredient = string_intern::Symbol<IngredientSymbol>;

#[derive(Clone, Copy)]
pub struct AllergenSymbol;
impl string_intern::Validator for AllergenSymbol {
    type Err = ::std::string::ParseError;
    fn validate_symbol(_: &str) -> Result<(), Self::Err> {
        Ok(())
    }
}
type Allergen = string_intern::Symbol<AllergenSymbol>;

// Food //

#[derive(Debug)]
pub struct Food {
    pub ingredients: Vec<Ingredient>,
    pub allergens: Vec<Allergen>,
}

impl Food {
    pub fn new(ingredients: Vec<Ingredient>, allergens: Vec<Allergen>) -> Food {
        Food {
            ingredients,
            allergens,
        }
    }
}

// Parser: //

pub mod parser {
    use super::{Allergen, Food, Ingredient};
    use nom::{
        bytes::complete::tag, character::complete::alpha1, character::complete::char,
        combinator::all_consuming, combinator::map, multi::many0, multi::separated_list1, IResult,
    };

    pub fn ingredient(input: &str) -> IResult<&str, Ingredient> {
        map(alpha1, |s: &str| s.parse::<Ingredient>().unwrap())(input)
    }

    pub fn allergen(input: &str) -> IResult<&str, Allergen> {
        map(alpha1, |s: &str| s.parse::<Allergen>().unwrap())(input)
    }

    pub fn line(input: &str) -> IResult<&str, Food> {
        let (input, ingredients) = separated_list1(tag(" "), ingredient)(input)?;
        let (input, _) = tag(" (contains ")(input)?;
        let (input, allergens) = separated_list1(tag(", "), allergen)(input)?;
        let (input, _) = tag(")")(input)?;
        Ok((input, Food::new(ingredients, allergens)))
    }

    pub fn alllines(input: &str) -> IResult<&str, Vec<Food>> {
        let (input, lines) = separated_list1(char('\n'), line)(input)?;
        let (input, _) = many0(char('\n'))(input)?;
        Ok((input, lines))
    }

    pub fn allinput(input: &str) -> IResult<&str, Vec<Food>> {
        all_consuming(alllines)(input)
    }
}

pub fn get_ingredients(foods: &[Food]) -> BTreeSet<Ingredient> {
    foods
        .iter()
        .flat_map(|f| f.ingredients.iter())
        .cloned()
        .collect::<BTreeSet<_>>()
}

pub fn get_allergens(foods: &[Food]) -> BTreeSet<Allergen> {
    foods
        .iter()
        .flat_map(|f| f.allergens.iter())
        .cloned()
        .collect::<BTreeSet<_>>()
}

pub fn check_partial_solution(foods: &[Food], sol: &BTreeMap<Ingredient, Allergen>) -> bool {
    for (ingredient, allergen) in sol.iter() {
        for food in foods {
            if food.allergens.contains(allergen) && !food.ingredients.contains(ingredient) {
                return false;
            }
        }
    }
    true
}

pub fn solve_rec(
    foods: &[Food],
    ingredients0: &BTreeSet<Ingredient>,
    allergens: &mut Vec<Allergen>,
    sol: &mut BTreeMap<Ingredient, Allergen>,
) -> bool {
    if allergens.is_empty() {
        return check_partial_solution(foods, sol);
    }
    let allergen = allergens.pop().unwrap();
    let mut ingredients = ingredients0.clone();
    for ingredient in ingredients0 {
        ingredients.remove(ingredient);
        sol.insert(ingredient.clone(), allergen.clone());
        if check_partial_solution(foods, sol) && solve_rec(foods, &ingredients, allergens, sol) {
            return true;
        }
        sol.remove(ingredient);
        ingredients.insert(ingredient.clone());
    }
    allergens.push(allergen);
    false
}

pub fn solve(foods: &[Food]) -> BTreeMap<Ingredient, Allergen> {
    let ingredients = get_ingredients(foods);
    let mut allergens = get_allergens(foods).iter().cloned().collect::<Vec<_>>();
    let mut sol = BTreeMap::<Ingredient, Allergen>::default();
    assert!(solve_rec(foods, &ingredients, &mut allergens, &mut sol));
    sol
}

// Process, etc //

fn process(mut bufin: impl BufRead) -> Result<usize> {
    let mut input = String::default();
    bufin.read_to_string(&mut input)?;
    let foods = parser::allinput(&input)
        .map_err(|e| anyhow!("error reading input: {:?}", e))?
        .1;
    let sol = solve(&foods);
    let mut count_safe = 0;
    for f in foods {
        for i in f.ingredients {
            if !sol.contains_key(&i) {
                count_safe += 1;
            }
        }
    }
    Ok(count_safe)
}

#[test]
fn test1() -> Result<()> {
    let input: &[u8] = b"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\ntrh fvjkl sbzzf mxmxvkd (contains dairy)\nsqjhc fvjkl (contains soy)\nsqjhc mxmxvkd sbzzf (contains fish)\n";
    eprintln!();
    // mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
    // trh fvjkl sbzzf mxmxvkd (contains dairy)
    // sqjhc fvjkl (contains soy)
    // sqjhc mxmxvkd sbzzf (contains fish)
    //
    // Free: kfcds, nhms, sbzzf, trh
    // dairy: mxmxvkd
    // fish: sqjhc
    // soy: fvjkl
    assert_eq!(process(input)?, 5);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
