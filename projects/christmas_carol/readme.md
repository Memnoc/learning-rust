## Christmas carol printer

Prints the lyrics to the Christmas carol "The Twelve Days of Christmas"

**The approach**
For the lyrics, you basically have 12 paragraphs, so we could treat them as blocks here and have 12 blocks, 1 paragraph each.

So we could build a match statements, match each case to a block, from 1 to 12, and break out when twelve is reached.

We have another function with a match statement for just the days, and that function is used to tell the previous one which day are we in.
This function also used the repetition to its advantage.
