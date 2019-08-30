using System;
using System.Collections.Generic;

namespace Berrysoft.Pages
{
    public class StringIgnoreCaseComparer : IEqualityComparer<string>
    {
        private StringIgnoreCaseComparer() { }

        private readonly static StringIgnoreCaseComparer instance = new StringIgnoreCaseComparer();
        public static IEqualityComparer<string> Instance => instance;

        public bool Equals(string x, string y) => x.Equals(y, StringComparison.OrdinalIgnoreCase);

        public int GetHashCode(string obj) => obj.ToLower().GetHashCode();
    }
}
