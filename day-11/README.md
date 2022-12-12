I was stuck on this, found the answer by looking at [this blog](https://nickymeuleman.netlify.app/garden/aoc2022-day11)

I had to get the least common multiple (LCM), and the item worry level is always moduled by this number after operations are executed on it. 

This way, if a number equals the LCM, the number divided by 0 will give true, making it divisible by any of the monkeys. And when it is passed into the next monkey, and if the monkey is multiplying, then it will stay 0 and meet that monkey's divisiblity condition. Otherwise if it is a plus, then the original condition would be LCM + num % divisble_number. and LCM % divisble num = 0, so this is the same as num % divisble_num as (A + B) % C = A%C + B%C. 

If it's over the LCM, this means that any value would, then the value would be LCM + num into num. For the same reason as before it would meet every individual monkey's conditions if they were true, as nothing would change from this value reassignment.

I also learned about Iter::drain(), and Iter::product(), so this way I could use less loops in my solutions in the future. With Operational Value, I could have done Add(val) or Times(val), rather than making multiple structs for it. Overall good learning experience.
