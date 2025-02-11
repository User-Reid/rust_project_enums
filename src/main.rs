#[derive(Debug)]
enum Tier {
    Gold,
    Silver,
    Platinum
}

#[derive(Debug)]
enum Subscription {
    Free,
    Basic(f64, u32),
    Premium {
        tier: Tier
    },
}


fn main() {
    Subscription::Free.summarize();
    Subscription::Basic(12.99, 12).summarize();
    Subscription::Premium { tier: Tier::Gold }.summarize();
}
