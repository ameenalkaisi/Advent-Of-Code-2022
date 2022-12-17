Q1: I go the parsing done, but I couldn't figure out the solution. I did recursion and it took too long even
on the sample. I tried to use caching but couldn't figure a good way to do it. I found
[Rusty Rob's code](https://gist.github.com/robert-king/76fb9c0b1ae1fedc0be6874239d2bde2) and adapted his solution onto
mine. Differences were that 1. He added the current pressure relived without any simulation. Mine I had a list where it kept all the
opened valves in the current state, and it would add all of them on each run, in his he took the current node and multiplied it by
minutes left, which is alot more efficient. And 2. The caching method he used included the opening path.

I didn't really understand why he didn't include ALL nodes, but only opened ones. However, when I tried pushing
the even nodes that weren't opened in the path, the program became too slow to run. Meaning cached values were
probalby not being used. I think the path was only being used as the list of opened nodes, rather than
a path that's being returned. So is a vector that is being used to store state of the current algorithm.
Also, if you try to use a HashMap, or a HashSet to store the opened nodes, it won't be storage inside of
the best_max_at or cache HashMap, since a HashSet or HashMap is not hashable.

Q2: Had no clue what to do, this solution is from
[Killavus](https://github.com/Killavus/Advent-of-Code-2022/blob/main/16-proboscidea-volcanium/src/main.rs).
At this point, this repo is just a blog. But then again I'm learning alot so it's useful.

1. Floyed Warshall algorithm: I thought the only way to get shortest path was through Dijkstra's or BFS, this way seems to be alot
   easier to implement than that or at least more straight forward. I just need a distance calculation method
   and to be able to enumerate through the entire graph. I thought using a HashMap<(String, String>, u32> was neat,
   I was thinking of doing a double vector but this is much better. Then, you can enumerate through all the
   valves' labels, and set valve to itself's distance to 0. Then every valve's leading valve could be
   set to 1 from both ways. Also, should remove nodes from self to self; this probably leads to infinite loops later.
2. Only searched a subset of nodes when going through the find_all_paths function. Those nodes were the non-zero ones.
   We still use the zero'd ones for calculating distance and such however.
3. The function all_relief_paths computes every possible from a given starting point, and gives the pressure relieved of that path, they are returned as a hash map.

   - There is pruning done when searching for nodes. It uses the distances that was calculated, so that if you are moving from node
     a to b, if the distance exceeds the allowed distance then it won't let you do that. Othewrise, if it is reachable, pick and
     recurse into it. The condition to go into a node is if the node either has been used in the current path inspected
     or if the distance to it is unreachable. Everything is returend through a mutable passed preference,
     which is a HashMap<String, i64>, however String represents set of all nodes used
   - Note that all_reliefs doesn't necessarily mean that it will START with "AA", so the solution has every non-zero flow-rate node
     call all_relief_paths, with "path so far" set to "AA" and distance set to "current valve" to "AA" (can be found by using Floyed
     Warshall structure computed earlier) at the start. It only computes EVERY PATH on a GIVEN STARTING NODE.
     - Further note, we don't need zero values, since the distances already calculated

4. Computing the elephant + person path, is done by comparing ALL paths that are disjoint from each other. So just pick the disjoint
   paths that produce the best result and that is the best path.
