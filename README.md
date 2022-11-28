# Syndr: Blog on the `#fediverse`

Syndr (pronounce like "cinder") is a work in progress. Any RSS or Atom feed becomes an entity that you can follow on the fediverse,
e.g., mastodon. You can subscribe to a feed by following, and you can comment on posts by replying. I hope
it'll effectively become a Disqus alternative where you can post comments via your mastodon account.

Where I think it'll be great:
* Disqus-style comments on blogs, but not bound to some random evilcorp
* Blogs, not microblogs — I mean, why can't full blogs be in the fediverse? Not second-class citizens, but top level social entities.
* Bots — It's pretty easy to produce and serve RSS or Atom. They're a single static XML document. Building a bot in the fediverse could be very easy.
* News aggregators — There's definitely overlap here. It would be easy to represent Lobsters or Hacker News on Synder in several ways (it's just RSS),
  but I think there's more interesting integrations.

Want to contribute? Too bad! It's just an idea at this point. Kidding, sort of, it's very early on right now. But if I did get a pull request, I'd probably merge it.
The current focus is fleshing out an idea and getting it to standup.

## ActivityPub
The plan is to use [RustyPub](https://github.com/hachyserve/rustypub) to implement the ActivityPub federation. The project seems too early right now to build on,
so I'm focused on nailing the syndication parts.

## Syndication
I'm using [syndication](https://crates.io/crates/syndication) for parsing RSS & Atom. I plan to build out the client part of the syndicaiton soon.

## Web App
Not much is built yet, but I'm using [axum](https://lib.rs/crates/axum) for the server. On the client, I want to use [htmx](https://htmx.org/) along with some
sort of [classless CSS](https://github.com/dbohdan/classless-css) framework. The hope is to keep the client as simple as possible for as long as I can manage.

