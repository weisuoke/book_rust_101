#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

struct PokerCard {
    suit: PokerSuit,
    value: u8,
}

fn main() {
    // 现在来创建 PokerSuit 枚举类型的两个成员实例
    // 我们通过 :: 操作符来访问 PokerSuit 下的具体成员，从代码可以清晰看出，heart 和 diamond 都是 PokerSuit 枚举类型，接着可以定义一个函数来使用它们：
    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;

    print_suit(heart);
    print_suit(diamond);

    // 这段代码很好的完成了它的使命，通过结构体 PokerCard 来代表一张牌，结构体的 suit 字段表示牌的花色
    // 类型是 PokerSuit 枚举类型，
    // value 字段代表扑克牌的数值
    let c1 = PokerCard {
        suit: PokerSuit::Clubs,
        value: 1,
    };

    let c2 = PokerCard {
        suit: PokerSuit::Diamonds,
        value: 12,
    };
}

fn print_suit(card: PokerSuit) {
    println!("{:?}", card);
}