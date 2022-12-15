Part 2 was pretty difficult for me since the approach I used in the first part made the second one harder. I used [this blog](https://nickymeuleman.netlify.app/garden/aoc2022-day14) for the second part. I learned a number of things.

1. Rust has a HashSet which is very good for when you need a "set theory" set, and not a vector, since it hashes things which makes searching faster. Elements in a hash set need to have Hash and Eq implemented.
2. flatten_map returns the result of the closure's into_iterator result. It turns out that every iterator also has into_iterator which helps make everything more confusing.

In my first appraoch, I tried to create lines to minimize the number of elements as obstacles, in case there is a lot of space between two different lines. Even with that in mind it ran very poorly. But in this approach, using a HashSet for searching makes the run very faster, because of HashSet searching, and no inefficiencies with searching every obstacle line. Also this is alot less complicated and generally makes more sense.
