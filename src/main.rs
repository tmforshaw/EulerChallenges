use rayon::prelude::*;

fn main() {
    println!("{}", amicable_numbers(10000));
}

#[allow(dead_code)]
fn is_prime(n: u32) -> bool {
    if n == 2 || n == 1 {
        return true;
    }

    for i in 3..n / 2 {
        if n % i == 0 {
            return false;
        }
    }

    true
}

#[allow(dead_code)]
fn find_nth_prime(n: u32) -> u32 {
    let mut prime_num = 2;
    let mut current_prime = 1;
    while prime_num <= n {
        current_prime += 2;
        if is_prime(current_prime) {
            prime_num += 1;
        }
    }

    current_prime
}

#[allow(dead_code)]
fn sum_primes_below_n(n: u32) -> u128 {
    let mut sum: u128 = 2;

    let mut current_prime = 1;
    while current_prime < n {
        current_prime += 2;
        if is_prime(current_prime) {
            sum += current_prime as u128;
        }
    }

    sum
}

#[allow(dead_code)]
fn sum_primes_between(lower: u32, upper: u32) -> u128 {
    let mut sum: u128 = 0;

    let mut current_prime = 1;

    if lower == 0 {
        sum = 2;
    } else if lower % 2 == 0 {
        current_prime = lower - 1
    } else {
        current_prime = lower - 2;
    }

    while current_prime <= upper - 2 {
        current_prime += 2;
        if is_prime(current_prime) {
            sum += current_prime as u128;
        }
    }

    sum
}

#[allow(dead_code)]
fn par_sum_primes_below_n(n: u32) -> u128 {
    let chunks = 32;

    let starting_vals: Vec<u32> = vec![0; chunks as usize]
        .iter()
        .enumerate()
        .map(|(i, _)| (n * (i + 1) as u32) / chunks)
        .collect();

    let sum = starting_vals
        .par_iter()
        .enumerate()
        .map(|(i, &val)| sum_primes_between(if i == 0 { 0 } else { starting_vals[i - 1] }, val))
        .fold(
            || 0,
            |mut acc, val| {
                acc += val;
                acc
            },
        )
        .reduce(
            || 0,
            |mut acc, val| {
                acc += val;
                acc
            },
        );

    sum
}

#[allow(dead_code)]
fn factorise(n: u32) -> Vec<u32> {
    let mut factors = vec![];

    for i in 1..=((n as f32).sqrt() as u32) {
        if n % i == 0 {
            factors.push(i);

            if n / i != i {
                factors.push(n / i);
            }
        }
    }

    factors.sort();
    factors
}

#[allow(dead_code)]
fn factors_more_than_n(n: u32) -> u32 {
    let mut factors = Vec::new();

    let mut i = 1;
    let mut triangular_num = 0;
    while factors.len() < n as usize {
        triangular_num += i;
        i += 1;

        factors = factorise(triangular_num);
    }

    triangular_num
}

#[allow(dead_code)]
fn longest_collatz_sequence(below: u32) -> u32 {
    let mut x: u128;

    let mut max_count = 0;
    let mut max_start = 0;

    for starting_val in 1..=(below as u128) {
        x = starting_val;
        let mut count = 0;

        while x != 1 {
            count += 1;

            if x % 2 == 0 {
                x /= 2;
            } else {
                x = 3 * x + 1
            }
        }

        if count > max_count {
            max_count = count;
            max_start = starting_val;
        }
    }

    max_start as u32
}

#[allow(dead_code, unused_variables)]
fn lattice_path_amounts(coord_to: (u32, u32)) -> u32 {
    // (2 * N) choose N

    // Permutations of right and down, to get to the end coordinate, double the N to get both diagonals (i think)

    todo!()
}

#[allow(dead_code)]
fn large_multiply(large_num: Vec<u32>, multiplicand: u32) -> Vec<u32> {
    let mut digits = large_num;
    let mut carry = 0;

    digits = digits
        .iter()
        .map(|&digit| {
            let product = digit.to_owned() * multiplicand + carry;

            carry = product / 10;

            product % 10
        })
        .collect::<Vec<_>>();

    while carry > 0 {
        digits.push(carry % 10);
        carry /= 10
    }

    digits
}

#[allow(dead_code)]
fn large_add(large_num: Vec<u32>, summand: u32) -> Vec<u32> {
    let mut digits = large_num;
    let mut carry = summand;

    digits = digits
        .iter()
        .map(|&digit| {
            let sum = digit + carry;

            carry = sum / 10;

            sum % 10
        })
        .collect::<Vec<_>>();

    while carry > 0 {
        digits.push(carry % 10);
        carry /= 10
    }

    digits
}

#[allow(dead_code)]
fn large_large_add(large_num: Vec<u32>, summand: Vec<u32>) -> Vec<u32> {
    let mut digits = large_num;
    let mut carry = 0;

    digits = digits
        .iter()
        .enumerate()
        .map(|(i, &left)| {
            let sum = left + if i < summand.len() { summand[i] } else { 0 } + carry;

            carry = sum.saturating_div(10);

            sum % 10
        })
        .collect::<Vec<_>>();

    while carry > 0 {
        digits.push(carry % 10);
        carry /= 10
    }

    digits
}

#[allow(dead_code)]
fn large_pow(base: u32, exp: u32) -> String {
    let mut digits = vec![base];

    for _ in 0..exp - 1 {
        digits = large_multiply(digits, base);
    }

    let digits_str = digits
        .iter()
        .rev()
        .map(|&digit| (digit as u8 + b'0') as char)
        .collect::<String>();

    digits_str
}

#[allow(dead_code)]
fn power_digit_sum(base: u32, exp: u32) -> u32 {
    large_pow(base, exp)
        .chars()
        .map(|c| (c as u8 - b'0') as u32)
        .sum()
}

#[allow(dead_code)]
fn number_to_word(n: u32) -> String {
    let digit_words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let tens_words = [
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];

    let ten_block_words = ["twen", "thir", "for", "fif", "six", "seven", "eigh", "nine"];

    let mut digits = Vec::new();
    let mut numerator = n;
    while numerator >= 1 {
        digits.push(numerator % 10);

        numerator /= 10;
    }

    digits
        .iter()
        .enumerate()
        .filter_map(|(i, &digit)| {
            if digit == 0 {
                None
            } else {
                match i {
                    0 => {
                        if digits.len() > 1 && digits[i + 1] == 0 && digit != 0 {
                            Some(format!("and {}", digit_words[digit as usize - 1]))
                        } else if digits.len() > 1 && digits[i + 1] == 1 && digit != 0 {
                            None
                        } else {
                            Some(digit_words[digit as usize - 1].to_string())
                        }
                    }
                    1 => {
                        if digit == 0 {
                            None
                        } else if digit == 1 {
                            Some(format!(
                                "{}{}",
                                if i != digits.len() - 1 { "and " } else { "" },
                                tens_words[digits[i - 1] as usize]
                            ))
                        } else {
                            Some(format!(
                                "{}{}ty",
                                if i != digits.len() - 1 { "and " } else { "" },
                                ten_block_words[digit as usize - 2]
                            ))
                        }
                    }
                    2 => Some(format!("{}-hundred", digit_words[digit as usize - 1])),
                    3 => Some(format!("{}-thousand", digit_words[digit as usize - 1])),
                    _ => None,
                }
            }
        })
        .rev()
        .fold(String::new(), |acc, string| {
            if acc.is_empty() {
                acc + string.as_str()
            } else {
                acc + " " + string.as_str()
            }
        })
}

#[allow(dead_code)]
fn letters_in_numbers_up_to(n: u32) -> u32 {
    let mut count = 0;
    for i in 1..=n {
        println!("{}", number_to_word(i));
        count += number_to_word(i).replace(" ", "").replace("-", "").len() as u32;
    }

    count
}

#[allow(dead_code)]
fn count_sundays_between(
    (from_day, from_month, from_year): (u32, u32, u32),
    (to_day, to_month, to_year): (u32, u32, u32),
) -> u32 {
    let day_of_week_on_0_0_1900 = 0;

    let mut day = 0;
    let mut month = 0;
    let mut year = 1900;

    let mut day_of_week = day_of_week_on_0_0_1900;
    let mut count = 0;

    let mut start_counting = false;

    loop {
        if year == to_year && month == to_month && day == to_day {
            break;
        }

        // If have went past starting date and day isn't sunday
        if year == from_year && month == from_month && day == from_day {
            start_counting = true;
        }
        if start_counting && day_of_week == 6 && day == 0 {
            // Only count sundays on first of the month
            count += 1;
        }

        println!("day: {day}, month: {month}, year: {year}\t day_of_week: {day_of_week}");

        // Iterate days, months, and years
        match month {
            3 | 5 | 8 | 10 => {
                // Thirty days
                if day < 29 {
                    day += 1
                } else {
                    day = 0;
                    month += 1;
                }
            }
            1 => {
                // Februray
                if day < 27 {
                    day += 1;
                } else {
                    // Leap year
                    if year % 4 == 0 && (year % 400 == 0 || year % 100 != 0) {
                        if day < 28 {
                            day += 1;
                        } else {
                            day = 0;
                            month += 1;
                        }
                    } else {
                        day = 0;
                        month += 1;
                    }
                }
            }
            0 | 2 | 4 | 6 | 7 | 9 | 11 => {
                // Thirty-one days
                if day < 30 {
                    day += 1;
                } else {
                    day = 0;

                    if month < 11 {
                        month += 1;
                    } else {
                        month = 0;
                        year += 1
                    }
                }
            }
            _ => {
                unreachable!()
            }
        }

        day_of_week = (day_of_week + 1) % 7;
    }

    count
}

#[allow(dead_code)]
fn factorial(n: u32) -> u32 {
    let mut product = 1;

    for i in 1..=n {
        product *= i;
    }

    product
}

#[allow(dead_code)]
fn large_factorial(n: u32) -> String {
    let mut product = vec![1];

    for i in 2..=n {
        product = large_multiply(product, i);
    }

    let product_str = product
        .iter()
        .rev()
        .map(|&digit| (digit as u8 + b'0') as char)
        .collect::<String>();

    product_str
}

#[allow(dead_code)]
fn distinct_powers(a_max: u32, b_max: u32) -> u32 {
    let mut pows = Vec::new();

    for a in 2..=a_max {
        for b in 2..=b_max {
            let a_pow_b = large_pow(a, b);

            if !pows.contains(&a_pow_b) {
                pows.push(a_pow_b.clone());
                // println!("{a_pow_b}");
            }
        }
    }

    pows.len() as u32
}

#[allow(dead_code)]
fn digit_cancelling_fractions() -> u32 {
    let mut fracs = Vec::new();
    for a in 1..=9 {
        for b in 0..=9 {
            for c in 1..=9 {
                for d in 0..=9 {
                    let real_result = (a * 10 + b) as f32 / (c * 10 + d) as f32;

                    if real_result.lt(&1f32) {
                        let cancelled_frac = if a == c {
                            if d > 0 {
                                (b, d)
                            } else {
                                continue;
                            }
                        } else if a == d {
                            (b, c)
                        } else if b == c {
                            if d > 0 {
                                (a, d)
                            } else {
                                continue;
                            }
                        } else if b == d {
                            (a, c)
                        } else {
                            continue;
                        };

                        let cancelled_result = cancelled_frac.0 as f32 / cancelled_frac.1 as f32;

                        if cancelled_frac.0 > 0
                            && cancelled_result.lt(&1f32)
                            && real_result == cancelled_result
                            // Remove trivial cases
                            && !(b == 0 && d == 0)
                        {
                            fracs.push(cancelled_frac);
                            println!("{a}{b}/{c}{d} = {cancelled_frac:?}");
                        }
                    }
                }
            }
        }
    }

    let product = fracs
        .iter()
        .fold((1, 1), |acc, f| (acc.0 * f.0, acc.1 * f.1));

    // Get simplified denominator
    product.1 / product.0
}

#[allow(dead_code)]
fn get_digits(n: u32) -> Vec<u32> {
    let mut numerator = n;

    let mut digits = Vec::new();

    while numerator > 0 {
        digits.push(numerator % 10);

        numerator /= 10;
    }

    digits
}

#[allow(dead_code)]
fn digit_factorials() {
    let mut sum_facts = Vec::new();

    for i in 3..=10000000 {
        let digits = get_digits(i);

        let sum_fact = digits.iter().map(|&digit| factorial(digit)).sum::<u32>();
        if sum_fact == i {
            sum_facts.push(i);
        }
    }

    println!("{sum_facts:?}: {}", sum_facts.iter().sum::<u32>());
}

#[allow(dead_code)]
fn factorial_digit_sum(n: u32) -> u32 {
    large_factorial(n)
        .chars()
        .map(|c| c as u32 - b'0' as u32)
        .sum()
}

#[allow(dead_code)]
fn champernown_constant() -> u32 {
    let mut champernown = String::new();
    for i in 0..=1e6 as u32 {
        champernown.push_str((i + 1).to_string().as_str());
    }

    (champernown.chars().next().unwrap() as u8 - b'0') as u32
        * (champernown.chars().nth(9).unwrap() as u8 - b'0') as u32
        * (champernown.chars().nth(99).unwrap() as u8 - b'0') as u32
        * (champernown.chars().nth(999).unwrap() as u8 - b'0') as u32
        * (champernown.chars().nth(9999).unwrap() as u8 - b'0') as u32
        * (champernown.chars().nth(99999).unwrap() as u8 - b'0') as u32
        * (champernown.chars().nth(999999).unwrap() as u8 - b'0') as u32
        * (champernown.chars().nth(999999).unwrap() as u8 - b'0') as u32
}

#[allow(dead_code)]
fn fibonacci_more_digits_than(n: u32) -> u32 {
    let mut f_n = vec![1];
    let mut f_prev_n = vec![1];

    let mut i = 2;
    while (f_n.len() as u32) != n {
        let current_f = f_n.clone();

        f_n = large_large_add(f_n, f_prev_n);
        f_prev_n = current_f;

        i += 1;
    }

    println!(
        "{}: {}\t{}",
        i,
        f_n.iter()
            .rev()
            .map(|&c| (c as u8 + b'0') as char)
            .collect::<String>(),
        f_n.len(),
    );

    i
}

#[allow(dead_code)]
fn amicable_numbers(less_than: u32) -> u32 {
    let mut amicable = Vec::new();

    for i in 1..less_than {
        let mut divisors = factorise(i);
        divisors.pop();
        let sum = divisors.iter().sum::<u32>();

        // Don't count perfect numbers
        if sum == i {
            continue;
        }

        let mut divisors_2 = factorise(sum);
        divisors_2.pop();
        let sum_2 = divisors_2.iter().sum::<u32>();

        // These are amicable numbers
        if sum_2 == i {
            amicable.push((i, sum));
        }
    }

    amicable.iter().map(|(num, _)| num).sum()
}
