trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct Post {
    title: String,
    author: String,
    content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("the author of post {} is {}", self.title, self.author)
    }
}

#[derive(Debug)]
struct Weibo {
    username: String,
    content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} published in Weibo: {}", self.username, self.content)
    }
}

// this function accepts any type that implements the specified trait
fn summary(input: &impl Summary) {// alternativelly fn summary<T:Summary>(input: &T)
    let output: String = input.summarize();
    println!("{}", output);
}

fn main() {
    let post: Post = Post {
        title: "Popular Rust".to_string(),
        author: "Sunface".to_string(),
        content: "Rust is awesome".to_string(),
    };
    let weibo: Weibo = Weibo {
        username: "Sunface".to_string(),
        content: "Weibo seems to be worse than Twitter".to_string(),
    };

    summary(&post);
    summary(&weibo);

    println!("{:?}", post);
    println!("{:?}", weibo);
}
