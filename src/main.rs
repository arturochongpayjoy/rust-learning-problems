fn main() {
    /*
        # Problem description
        Write a program that takes an array denoting the daily stock price, 
        and returns the maximum profit that can be made by buying and then selling 
        one share of that stock.

        310, 315, 275, 295, 260, 270, 290, 230, 255, 250

        Hint: identifying the minimum and maximum is not enough 
        since the minimum may appear after the maximum price.
        
        # Solution
        The answer is 230, because 230 - 255 = 25 profit.
        
        Array must have length n >= 2. You only realize gains or losses when you sell,
        and this implies that there must be change in price.
        
        For any given array that satisfies above condition, 
        the number of possible solutions is n - 1, regardless of if array length is
        even or odd.
        
        Consider n = 2
        [1, 2]
        There is only one possible combination, which is [1, 2]
        This satisfies n - 1
        n = 2 items in array
        n - 1 = 1 combination
        
        Consider n = 3
        [1, 2, 3]
        There are only two possible combinations, which are [1, 2] and [2, 3]
        This satisfies n - 1
        n = 3 items in array
        n - 1 = 2 combinations
        
        Of these n - 1 combinations, where every combination is in the form of [a, b], only
        combinations where a < b are valid, because you realize a gain. If a > b, you realize
        a loss.

        Consider [1, 2], where a < b
        1 - 2 = -1, a gain of one unit.
        
        Consider [2, 1], where a > b
        2 - 1 = 1, a loss of one unit.
        
        Let x be a subset of n - 1, where x is a valid combination.
        The solution is thus whichever combination in x where a - b produces the lowest number. 
    */
    let stock_prices = [310, 315, 275, 295, 260, 270, 290, 230, 255, 250];
    let mut profit = 0;
    let mut winning_index = 0;
    let mut possible_combinations = Vec::new();

    for index in 0..stock_prices.len() - 1 {
        let subset = &stock_prices[index..index + 2]; // +2 because upper bound is non-inclusive
        possible_combinations.push(subset);
        profit = if subset[0] - subset[1] < profit {
            winning_index = index;
            subset[0] - subset[1]
        } else {
            profit
        }
    }
    println!("Biggest profit: {}", profit);
    println!("Buy on day: {}", winning_index);
    println!(
        "Winning combination: {}, {}", 
        stock_prices[winning_index], 
        stock_prices[winning_index + 1]
    );
    
    println!("Possible combinations: ");
    for (index, combination) in possible_combinations.iter().enumerate() {
        println!(
            "Day {}: {} - {} = {}",
            index,
            combination[0], 
            combination[1],
            combination[0] - combination[1]
        );
    }
}
