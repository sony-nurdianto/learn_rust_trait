trait Summary {
    fn summarize(&self) -> String;

    fn summarize_author(&self) -> String {
        format!("Read More from {}...", self.summarize())
    }

    fn notif(&self) {
        println!("Ping Notif From")
    }
}
//this code is not clean it for learning purpose
trait Detail {
    fn show_user(&self) -> &str;
    fn show_content(&self) -> &str;
}

struct NewsLetter {
    headline: String,
    author: String,
    location: String,
    content: String,
}

impl Detail for NewsLetter {
    fn show_user(&self) -> &str {
        &self.author
    }

    fn show_content(&self) -> &str {
        &self.content
    }
}

impl Summary for NewsLetter {
    fn summarize(&self) -> String {
        format!(
            "Our NewsLetter Today {}\nWrite by {}\n\n{}\n\n{}",
            self.headline, self.author, self.content, self.location
        )
    }

    fn summarize_author(&self) -> String {
        format!("Read More from {}...", self.author)
    }

    fn notif(&self) {
        println!("Notify Ping from {}", self.author);
    }
}

struct Twitter {
    username: String,
    content: String,
    reply: Vec<String>,
    retweet: bool,
}

impl Detail for Twitter {
    fn show_user(&self) -> &str {
        let username: &str = &self.username;
        println!("{}", username);
        &self.username
    }
    fn show_content(&self) -> &str {
        let content: &str = &self.content;
        println!("{}", content);
        &self.content
    }
}

impl Summary for Twitter {
    fn summarize(&self) -> String {
        let collec_reply: String = self.reply.join("\n");
        format!(
            "{}\n{}\n\nreply:\n{}\n\nretweet:{}",
            self.username, self.content, collec_reply, self.retweet
        )
    }
    fn summarize_author(&self) -> String {
        format!("Read More from {}...", self.username)
    }

    fn notif(&self) {
        println!("Notify Ping from {}", self.username)
    }
}

//for learning purpose
fn notify(item: &(impl Summary + Detail)) {
    item.notif();
    item.show_user();
    item.show_content();
}

//for learning purpose
fn duplicate_notify<S, D>(s: &S, d: &D)
where
    S: Summary,
    D: Detail,
{
    s.notif();
    d.show_user();
    d.show_content();
}

fn main() {
    let newsletter_today: NewsLetter = NewsLetter {
        author: String::from("Stereo Heart"),
        content: String::from(
            "My Heart so stereo it's beat for you so listen close hear my thoug in every note ooh",
        ),
        location: String::from("New York"),
        headline: String::from("My Heart so stereo"),
    };
    let twitter_from_adele: Twitter = Twitter {
        username: String::from("Adele"),
        content: String::from("Go Easy On me"),
        reply: vec![
            String::from("So Sad 😢"),
            String::from("Please go easy on me baby"),
        ],
        retweet: true,
    };

    println!("{}\n", newsletter_today.summarize());
    println!("{}\n", twitter_from_adele.summarize());

    let more_newsletter_today: NewsLetter = NewsLetter {
        author: String::from("Lea"),
        content: String::from("Und alle meine Freunde\nFinden, dass ich leiser bin\nDass ich leiser bin\nLeiser, seit ich bei dir bin\nAlle meine Freunde\nFragen, ob ich glücklich bin\nWeil ich leiser bin\nLeiser, seit ich bei dir bin",
        ),
        location: String::from("München"),
        headline: String::from("ich leiser bin"),
    };
    println!("{}\n", more_newsletter_today.summarize());
    println!("{}", more_newsletter_today.summarize_author());

    notify(&more_newsletter_today);
    println!("xxxxxxxxxxxxxxxxxxxxxx Divider xxxxxxxxxxxxxxxxxxxx");
    duplicate_notify(&more_newsletter_today, &twitter_from_adele);
}
