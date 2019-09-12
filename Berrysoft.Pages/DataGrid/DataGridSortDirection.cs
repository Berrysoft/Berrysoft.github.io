using System;
using System.Collections.Generic;
using System.Linq;

namespace Berrysoft.Pages.DataGrid
{
    public enum DataGridSortDirection
    {
        None,
        Ascending,
        Descending
    }

    static class DataGridSortHelper
    {
        public static IEnumerable<T> OrderBy<T, TKey>(this IEnumerable<T> source, Func<T, TKey> keySelector, DataGridSortDirection direction)
        {
            return direction switch
            {
                DataGridSortDirection.Ascending => source.OrderBy(keySelector, UniversalComparer<TKey>.Default),
                DataGridSortDirection.Descending => source.OrderByDescending(keySelector, UniversalComparer<TKey>.Default),
                _ => source
            };
        }
    }

    class UniversalComparer<T> : IComparer<T>
    {
        public static UniversalComparer<T> Default { get; } = new UniversalComparer<T>();

        private UniversalComparer() { }

        public int Compare(T x, T y)
        {
            if (x is string)
            {
                return StringComparer.OrdinalIgnoreCase.Compare(x, y);
            }
            else
            {
                return Comparer<T>.Default.Compare(x, y);
            }
        }
    }
}
