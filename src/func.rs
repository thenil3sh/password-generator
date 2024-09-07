use rand::seq::SliceRandom;

#[derive(Debug)]
enum Arr {
    Larray,
    CapLarray,
    Narray,
    Sarray,
}
const LARRAY: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

const WARRAY: [&str; 451] = [
    "the",
    "be",
    "to",
    "of",
    "and",
    "a",
    "in",
    "that",
    "have",
    "I",
    "it",
    "for",
    "not",
    "on",
    "with",
    "he",
    "as",
    "you",
    "do",
    "at",
    "this",
    "but",
    "by",
    "from",
    "they",
    "we",
    "say",
    "her",
    "she",
    "or",
    "an",
    "will",
    "my",
    "one",
    "all",
    "would",
    "there",
    "their",
    "what",
    "so",
    "up",
    "out",
    "if",
    "about",
    "who",
    "get",
    "which",
    "go",
    "me",
    "make",
    "can",
    "like",
    "time",
    "no",
    "just",
    "him",
    "know",
    "take",
    "people",
    "into",
    "year",
    "your",
    "good",
    "some",
    "could",
    "them",
    "see",
    "other",
    "than",
    "then",
    "now",
    "look",
    "only",
    "come",
    "its",
    "over",
    "think",
    "also",
    "back",
    "after",
    "use",
    "two",
    "how",
    "our",
    "work",
    "first",
    "well",
    "way",
    "even",
    "new",
    "want",
    "because",
    "any",
    "these",
    "give",
    "day",
    "most",
    "us",
    "where",
    "down",
    "out",
    "up",
    "same",
    "such",
    "here",
    "find",
    "place",
    "why",
    "those",
    "her",
    "little",
    "own",
    "other",
    "such",
    "still",
    "through",
    "between",
    "life",
    "being",
    "few",
    "might",
    "again",
    "few",
    "leave",
    "might",
    "right",
    "always",
    "part",
    "great",
    "last",
    "number",
    "own",
    "both",
    "high",
    "every",
    "man",
    "every",
    "under",
    "most",
    "after",
    "things",
    "might",
    "thing",
    "might",
    "away",
    "next",
    "work",
    "night",
    "those",
    "off",
    "under",
    "long",
    "see",
    "own",
    "find",
    "right",
    "end",
    "work",
    "long",
    "even",
    "own",
    "only",
    "through",
    "work",
    "again",
    "under",
    "both",
    "start",
    "good",
    "both",
    "two",
    "time",
    "year",
    "people",
    "way",
    "day",
    "man",
    "thing",
    "woman",
    "life",
    "child",
    "world",
    "school",
    "state",
    "family",
    "student",
    "group",
    "country",
    "problem",
    "hand",
    "part",
    "place",
    "case",
    "week",
    "company",
    "system",
    "program",
    "question",
    "work",
    "government",
    "number",
    "night",
    "point",
    "home",
    "water",
    "room",
    "mother",
    "area",
    "money",
    "story",
    "fact",
    "month",
    "lot",
    "right",
    "study",
    "book",
    "eye",
    "job",
    "word",
    "business",
    "issue",
    "side",
    "kind",
    "head",
    "house",
    "service",
    "friend",
    "father",
    "power",
    "hour",
    "game",
    "line",
    "end",
    "member",
    "law",
    "car",
    "city",
    "community",
    "name",
    "president",
    "team",
    "minute",
    "idea",
    "kid",
    "body",
    "information",
    "back",
    "parent",
    "face",
    "others",
    "level",
    "office",
    "door",
    "health",
    "person",
    "art",
    "war",
    "history",
    "party",
    "result",
    "change",
    "morning",
    "reason",
    "research",
    "girl",
    "guy",
    "moment",
    "air",
    "teacher",
    "force",
    "education",
    "foot",
    "boy",
    "age",
    "policy",
    "process",
    "music",
    "market",
    "sense",
    "nation",
    "plan",
    "college",
    "interest",
    "death",
    "experience",
    "effect",
    "use",
    "class",
    "control",
    "care",
    "field",
    "development",
    "role",
    "effort",
    "rate",
    "heart",
    "drug",
    "show",
    "leader",
    "light",
    "voice",
    "wife",
    "police",
    "mind",
    "price",
    "report",
    "decision",
    "son",
    "view",
    "relationship",
    "town",
    "road",
    "arm",
    "difference",
    "value",
    "building",
    "action",
    "model",
    "season",
    "society",
    "tax",
    "director",
    "position",
    "player",
    "record",
    "paper",
    "space",
    "ground",
    "form",
    "event",
    "official",
    "matter",
    "center",
    "couple",
    "site",
    "project",
    "activity",
    "star",
    "table",
    "need",
    "court",
    "oil",
    "situation",
    "cost",
    "industry",
    "figure",
    "street",
    "image",
    "phone",
    "data",
    "picture",
    "practice",
    "piece",
    "land",
    "product",
    "doctor",
    "wall",
    "patient",
    "worker",
    "news",
    "test",
    "movie",
    "north",
    "love",
    "support",
    "technology",
    "step",
    "baby",
    "computer",
    "type",
    "attention",
    "film",
    "tree",
    "source",
    "organization",
    "hair",
    "look",
    "science",
    "owner",
    "brother",
    "window",
    "skill",
    "sport",
    "board",
    "subject",
    "officer",
    "rest",
    "range",
    "trade",
    "past",
    "goal",
    "amount",
    "audience",
    "letter",
    "risk",
    "character",
    "growth",
    "degree",
    "attack",
    "region",
    "television",
    "box",
    "tv",
    "training",
    "style",
    "camera",
    "horse",
    "painting",
    "patient",
    "truck",
    "debt",
    "staff",
    "paper",
    "animal",
    "dog",
    "bird",
    "cut",
    "food",
    "agreement",
    "leader",
    "salt",
    "character",
    "boat",
    "winter",
    "tree",
    "voice",
    "skin",
    "wind",
    "track",
    "language",
    "chance",
    "brochure",
    "map",
    "radio",
    "environment",
    "region",
    "self",
    "show",
    "fire",
    "shot",
    "growth",
    "second",
    "pollution",
    "science",
    "nature",
    "product",
    "television",
    "theory",
    "month",
    "church",
    "pupil",
];

const CAPLARRAY: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

const NARRAY: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
const SARRAY: [char; 16] = [
    '!', '#', '$', '%', '&', '*', '+', '-', '.', ':', '=', '?', '@', '^', '_', '~',
];

pub fn charray(len: u8, capl: bool, n: bool, s: bool) -> String {
    let mut oreo = String::new();
    let mut chances: Vec<Arr> = vec![Arr::Larray];
    if capl {
        chances.push(Arr::Larray)
    }
    if n {
        chances.push(Arr::Narray)
    }
    if s {
        chances.push(Arr::Sarray)
    }

    for _i in 0..len {
        let chartype = match chances.choose(&mut rand::thread_rng()) {
            Some(x) => x,
            None => panic!("chartype variable got None value :( sad"),
        };
        let char = match chartype {
            Arr::Larray => match LARRAY.choose(&mut rand::thread_rng()) {
                Some(x) => {
                    if capl {
                        chances.push(Arr::CapLarray)
                    }
                    if n {
                        chances.push(Arr::Narray)
                    }
                    if s {
                        chances.push(Arr::Sarray)
                    }
                    *x
                }
                None => panic!("Looks like larray is empty or doesn't exist"),
            },
            Arr::CapLarray => match CAPLARRAY.choose(&mut rand::thread_rng()) {
                Some(x) => {
                    chances.push(Arr::Larray);
                    if n {
                        chances.push(Arr::Narray)
                    }
                    if s {
                        chances.push(Arr::Sarray)
                    }
                    *x
                }
                None => panic!("Looks like caplarray is empty or doesn't exist"),
            },
            Arr::Narray => match NARRAY.choose(&mut rand::thread_rng()) {
                Some(x) => {
                    chances.push(Arr::Larray);
                    if capl {
                        chances.push(Arr::CapLarray)
                    }
                    if s {
                        chances.push(Arr::Sarray)
                    }
                    *x
                }
                None => panic!("Looks like narray is empty or doesn't exist"),
            },
            Arr::Sarray => match SARRAY.choose(&mut rand::thread_rng()) {
                Some(x) => {
                    chances.push(Arr::Larray);
                    if capl {
                        chances.push(Arr::CapLarray)
                    }
                    if n {
                        chances.push(Arr::Narray)
                    }
                    *x
                }
                None => panic!("Looks like sarray is empty or doesn't exist"),
            },
        };

        oreo.push(char);
    }
    //println!("{:?}", chances);
    oreo
}

pub fn warray(len: u8, capital: bool, numbers: bool, symbols: bool, delimeter: char) -> String {
    let mut oreo = String::new();
    let mut i: u8 = 0;
    let mut chances = vec![];
    if capital {
        chances.push(Arr::Larray)
    };
    if numbers {
        chances.push(Arr::Narray)
    };
    if symbols {
        chances.push(Arr::Sarray)
    };

    while i < len {
        let word = WARRAY.choose(&mut rand::thread_rng()).unwrap();
        if i + 1 < len && i != 0 {
            oreo.push(delimeter);
            i += 1;
        } else if i == 0 {
        } else {
            break;
        }

        if capital {
            oreo.push_str(&camelcased(word));
        } else {
            oreo.push_str(word);
        }
        i += word.len() as u8;

        if numbers && i < len {
            oreo.push(num_char());
            i += 1;
        }
    }
    oreo
}

fn camelcased(word: &str) -> String {
    let mut flag = true;
    let mut camel = String::new();
    for index in 0..word.len() {
        if flag {
            camel.push(word.chars().nth(index).unwrap().to_ascii_uppercase());
            flag = false;
            continue;
        }
        camel.push(word.chars().nth(index).unwrap());
    }
    camel
}

fn num_char() -> char {
    let num = NARRAY.choose(&mut rand::thread_rng()).unwrap();
    *num
}
