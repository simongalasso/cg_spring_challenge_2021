# CG 2021 Spring Challenge PostMortem

## Stats
* Overall rank: <strong>100</strong>th out of <strong>6 867</strong> players
* Tech rank (Rust): <strong>6</strong>th out of <strong>202</strong> players
* Team rank (42 school): <strong>3</strong>th out of <strong>213</strong> players
* Country rank (France): <strong>36</strong>th out of <strong>2627</strong> players

## The challenge
For this challenge the goal was to make an AI that is able to win a match on a Photosynthesis board game, <a href=https://www.codingame.com/challengereport/3371192040f5be56acaf8c1ad3e8d4f347bb82da>official rules here</a>
<div><img src="demo.gif" width="300"/></div>

## My Progression
### Wood2
#### Rules
* There is an hex map, of 37 cells that can be empty or filled with a tree owned by one of the two players, every cells have a richness value that depends of theirs positions. The game also old a value called nutrients, that start by 20.
* The game takes place in several "days" themself splitted by turns, one action can be executed per turn, and the day end when every players WAIT, for this league the game ends at the end of the day.
* Every players have an amount of sun points that are the money used to execute actions.
* For this league, only two actions are available : COMPLETE a tree (that cost 4 sun points) and WAIT for the next day.
* COMPLETE a tree give score points (the amount of nutrients + a value based on the richness of the cell), it also decrease the nutrients value for every players.
* At the end of the game, the number of sun / 3 is added to the player score.
* The player with the higher score at the end win.
* In case of equality, the player with the most number of trees left on the map win.
#### My strategy
Find the trees that are on the most rich cells and COMPLETE them -> Wood1
### Wood1 League
#### Rules
* The day ends now after day 6.
* Tree can have different sizes : 1, 2 and 3, only size_3 can be COMPLETED
* Another action is available : GROW, that increase the size of a tree, it cost the number of trees of the new size that I already have + a value base on the size of that tree.
* Every day, each tree give an amount of sun points based on their sizes.
* Only size_3 trees can be COMPLETED.
#### My strategy
* Implemented my old friend the IDDFS of the last challenge <a>https://github.com/sgalasso42/cg_fall_challenge_2020</a> with a random heuristic -> Bronze
### Bronze League
#### Rules
* The game now ends at 24 days.
* It is now possible to SEED, this action cost the number of seed already I already have on the map, and this size_0 tree new does not give any sun points.
* Tree can SEED at a distance of their size (base on manhattan distance), size_2 tree can seed a 2 cells, size_3 at 3 cells...
* Another feature appeared : shadow, the sun has an orientation that change every day on the 6-day week, so the tree make shadow with a size and a stength taht depends on the tree size that made it. Tree of size 3 make 3 cells long shadow, size_2 make a 2 cells long shadow... If a tree is shadowed by another, it does not give sun. Trees can be shadowed by tree with size equal or bigger than itself.
#### My strategy
At this time I was simulating everything but shadows. A problem occured, there were too much available actions per turn, so the graph was too wide to go at an interesting depth with my IDDFS. I then tried an MCTS for a while, it did not work as expected, I was not prunning enough and when there were too much available actions at a node, my number of simulation was really not enough to have a good average score on higher nodes.<br>
I then focused improving evaluation function, I also fixed bugs, cleaned code, but the day of the Iron league were approaching and my rank was so low that at the end I took back my IDDFS with my new evaluation function and I passed in Silver when it oppened !
### Silver League
* No new rules, the Bronze ones where the last to change
#### My strategy
I realized that most of the top rank on that league were not using graph search algorithm, they were juste evaluating an action on the depth-1 for a given state. If they were upper without simulating that means my evaluation function was really bad, so I spend all my time during this league on that, I started handling shadows and completely refactor my evaluation, Gold oppened, I was 450th and they took 650 players, nice.
### Gold League
My function was still full of arbitrary values, with some good ideas shared on our team chanel and a little thought I managed to nicely refactor it, this is what my strategy was to pass into Legend league :
```
// sun_rate = sun_production - shadow_cost + opponent_shadow_cost

if available COMPLETE actions
    && my number of size_3 < 20 - day
    && score given by the tree is higher than 0
{
    COMPLETE trees that have the lowest next_day_sun_rate (week_sun_rate in case of equality)
} else if available GROW actions
    && nb day left > nb of day necessary to complete the tree
    && next_day_sun_rate > current_next_day_sun_rate
{
    GROW tree with better next_day_sun_rate (week_sun_rate in case of equality)
} else if available SEED actions
    && nb day left > nb of day necessary to complete the tree
{
    SEED on cell that has the less shadow on a week (richness checked in case of equality)
} else WAIT
```
Calculating the sun_rate for the available GROW actions, stopping using SEED, GROW and COMPLETE when it was useless at endgame and calculating the bound day for COMPLETE instead of a fixed value were the features that took me to LEGEND.

### Legend League
I struggled all the last nigth before the challenge ends twicking values to find something to get into top 100, 2 hours before the end around 8am I found something, for GROW, instead of calculating the absolute sun_rate, I calculated the difference between the current one and the potential new one and ! It made the difference and I finished 100th !

## Special thanks

I would thanks <a href="https://github.com/mapapin">mapapin<a/>, <a href="https://github.com/kh42z">Kh4z<a/>, <a href="https://github.com/pde-bakk">peerdb<a/>, <a href="https://github.com/Rush-iam?tab=repositories">nGragas<a/> and <a href="https://github.com/rmarracc">Shin0auBSQ<a/> who helped me to get out of my frequent struggles I had on that challenge ! Also thanks to <a href="https://www.codingame.com/home">CodinGame</a> team for making such good contests !
