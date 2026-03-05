
pub fn get_amount_out(
    amount_in:i128,
    reserve_in:i128,
    reserve_out:i128
)->i128{
    if amount_in <= 0{
        panic!("Invalid input amount")  
    }
    if reserve_in <= 0 || reserve_out <= 0{
        panic!("Invalid reserves")
    }

    let amount_in_with_fee = amount_in * 997;
    let numerator = amount_in_with_fee * reserve_out;
    let denominator = reserve_in * 1000 + amount_in_with_fee;
    numerator / denominator 
}

pub fn sqrt(value:i128) ->i128{
    let mut z = (value + 1) / 2;
    let mut y = value;
    while z < y {
        y = z;
        z = (value / z + z)  / 2;
    }
    y
}