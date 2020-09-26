using System;
using System.Collections.Generic;
using System.Linq;

namespace Berrysoft.Pages.KanaLearn
{
    static class RandomHelper
    {
        public readonly static Random Generator = new Random();

        public static IEnumerable<T> Random<T>(this IEnumerable<T> source)
        {
            List<T> s = source.ToList();
            int count = s.Count;
            for (int i = 0; i < count; i++)
            {
                int index = Generator.Next(s.Count);
                yield return s[index];
                s.RemoveAt(index);
            }
        }
    }
}
