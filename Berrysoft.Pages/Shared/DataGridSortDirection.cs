using System;
using System.Collections.Generic;
using System.Linq;

namespace Berrysoft.Pages.Shared
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
                DataGridSortDirection.Ascending => source.OrderBy(keySelector),
                DataGridSortDirection.Descending => source.OrderByDescending(keySelector),
                _ => source
            };
        }
    }
}
