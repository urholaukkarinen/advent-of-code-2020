#include <fstream>
#include <iostream>
#include <sstream>
#include <map>

int main()
{
   std::ifstream infile("input.txt");

   std::map<char, int> map;

   int part_one = 0;
   int part_two = 0;
   int group_size = 0;

   while (true)
   {
      std::string line;
      if (infile.good())
      {
         std::getline(infile, line);
      }

      if (line.empty())
      {
         part_one += map.size();
         for (auto it = map.begin(); it != map.end(); it++)
         {
            if (it->second == group_size)
            {
               part_two += 1;
            }
         }
         map.clear();
         group_size = 0;

         if (!infile.good())
         {
            std::cout << part_one << '\n'
                      << part_two << '\n';
            break;
         }
         else
         {
            continue;
         }
      }

      group_size += 1;

      for (char const &c : line)
      {
         auto it = map.find(c);
         if (it != map.end())
         {
            it->second++;
         }
         else
         {
            map.insert(std::pair<char, int>(c, 1));
         }
      }
   }
}