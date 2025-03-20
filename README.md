# are we gonna win the league?
written by Tierney McBride
for CS 523 Programming in Rust at PSU, instructor Bart Massey

## Summary
Web app written in Rust to estimate the probability a given team
will finish in a given place in the English Premier League season
by simulating the remainder of the season using a Monte Carlo simulation.

Once built, the app is setup to be hosted locally, with the dream to eventually have an externally accessible site with up-to-date data.

## Process
The simplest aspects of the program, as it turned out, were building out the library of functions needed to run the simulation and implementing the threads. Modifying the program into a web app was more complex than I imagined, but quite a lot of fun. I can't seem to get the interface to acknowledge and employ my stylesheet, but that is of little importance.

The primary stumbling block I ran up against related to my intention to employ API integrations to retrieve the current Premier League data required to produce accurate results at any point in the season. As is, the program will provide accurate responses until Premier League play resumes on April 1, 2025. Despite various attempts, I was unable to find an API that would provide current season standings and fixture lists at a free tier level. I then spent far too long attempting to build my own scraper, before abandoning the venture in frustration and resorting to typing up my own json files containing the data as of the week of finals and the week following simply to be able to demonstrate the functionality more representative of the Rust I've learned and been working on.

I have found reading Rust documentation surprisingly interesting and useful. I found that nearly all the crates I employed had extensive and clear documentation, as well as helpfully demonstrative examples.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
