# [Day 7 The Great Log Hunt](https://www.rustfinity.com/practice/rust/challenges/aor-2024-7/description)

The tension in the North Pole Dev room had not dissipated. Santa’s absence loomed large, and Blitzen had clearly let the power go to his antlers. The elves were beginning to mutter about mutiny—especially after Blitzen had loudly declared that grep was overrated and **"real devs"** write their own search tools.

_“We need a better log system,”_ Blitzen announced, pacing in front of the DevOps board like a caffeinated startup founder. _“I’m tired of manually combing through logs! It’s time we automate this.”_

Prancer peeked up from their desk. _“Can’t we just pipe the logs into **grep** like everyone else?”_

Blitzen’s glare could have melted the polar ice caps. _“Prancer, if you’re going to suggest mediocre solutions, you can go back to working in Node.js.”_

Prancer recoiled, whispering, _“Too far, Blitzen. Too far.”_

## A LogQuery Tool

Blitzen wanted a log **search tool** so advanced that even Santa would call it “blitzening fast.” Logs were piling up from every North Pole subsystem—Toy Tracker 3000, SleighOS, and even Reindeer AI. The elves needed to find specific entries without scrolling for hours.

_“You!”_ Blitzen pointed at **Frostbyte**, the elf known for typing faster than a Model M keyboard. _“You’re going to write a `LogQuery` struct in Rust that can search through our logs.”_

Frostbyte cracked his knuckles, opened NeoVim, and got to work.

But he needs your help to be saved from Blitzen’s sass and implement the `LogQuery` struct with its `search` method?

## The Requirements:

Here’s what Frostbyte must implement:

- **LogQuery struct**: This struct should hold a reference to a slice of `String` logs.
- **new()** Create an [associated function](https://www.rustfinity.com/learn/rust/structs/implementing-methods#associated-functions) named new that accepts a reference to a Vec<String> and returns a `LogQuery`.
- **Search method**: A method named `search` that:
  1. Returns a `Vec` of [references](https://www.rustfinity.com/learn/rust/ownership/borrowing) to strings containing the `keyword`.
  2. Should work even if the logs are empty.
  3. Special characters in the `keyword` must be handled properly.

## Hints

If you're stuck or need a starting point, here are some hints to help you along the way!

- Your LogQuery struct will likely hold a &'a Vec<String>.

```
pub struct LogQuery<'a> {
  logs: &'a Vec<String>,
}
```

- The `new` function should accept a reference to a `&'a Vec<String>` and return a `LogQuery`.
- The `search` method should accept a `&self` and a `keyword: &str` parameter.
- To return references to the logs, you can use `-> Vec<&'a String>`.
- Implement `search` by iterating over self.logs. e.g. `self.logs.iter()`.
- Use `filter` and provide a closure. e.g. `filter(|log| {})`.
- Use `contains` to check if a log contains the `keyword`. e.g. `log.contains(keyword)`.
- Use `collect` to convert the iterator back to a `Vec`. e.g. `collect::<Vec<_>>()`.