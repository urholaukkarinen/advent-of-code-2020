import std.stdio, std.string, std.algorithm, std.file;

void main()
{
   auto input = readText("input.txt");
   auto width = input.indexOf("\n");
   input = input.replace("\n", "");
   auto height = input.length / width;

   long partOne = 0;
   long partTwo = 1;

   foreach (slope; [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]]) {
      int y = 0;
      int x = 0;
      int trees = 0;

      while (y < height) {
         if (input[y * width + (x % width)] == '#') {
            trees++;
         }

         x += slope[0];
         y += slope[1];
      }

      if (slope == [3, 1]) {
         partOne = trees;
      }

      partTwo *= trees;
   }

   writeln(partOne);
   writeln(partTwo);
}