pub fn dp_rec_mc(amount: u32) -> u32 {
    let mut total = 0;
    let mut amount = amount;
    const CURRENCY: [u32; 8] = [1u32, 2, 5, 10, 20, 30, 50, 100];

    while amount != 0 {
        let mut prev = 1u32;
        for currency in CURRENCY {
            if currency <= amount {
                prev = currency;
            } else {
                break;
            }
        }
        amount -= prev;
        total += 1;
    }

    total
}
