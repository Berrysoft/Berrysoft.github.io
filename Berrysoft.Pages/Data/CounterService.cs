using System;
using System.Linq;

namespace Berrysoft.Pages.Data
{
    public interface ICounterService
    {
        int TotalCount { get; }
        int DistinctCount { get; }
        int SentenceCount { get; }
        int? AverageCount { get; }

        string Text { get; set; }
    }
    
    public class CounterService : ICounterService
    {
        public int TotalCount { get; private set; }

        public int DistinctCount { get; private set; }

        public int SentenceCount { get; private set; }

        public int? AverageCount { get; private set; }

        private string text;
        public string Text
        {
            get => text;
            set
            {
                if (text != value)
                {
                    text = value;
                    CalculateCounts();
                }
            }
        }

        static readonly char[] WordSeparator = new char[] { ' ', '\r', '\n', '.', ',', '!', '?', ':', ';', '(', ')' };
        static readonly char[] WordTrimmer = new char[] { '-', '\'', '\"' };
        static readonly string[] SentenceSeparator = new string[] { "\r", "\n", ". ", "! ", "? " };

        private void CalculateCounts()
        {
            var words = text
                .Split(WordSeparator, StringSplitOptions.RemoveEmptyEntries)
                .Select(w => w.Trim(WordTrimmer))
                .Where(w => !string.IsNullOrEmpty(w))
                .ToArray();
            TotalCount = words.Length;
            DistinctCount = words.Distinct(StringComparer.OrdinalIgnoreCase).Count();
            SentenceCount = text
                .Split(SentenceSeparator, StringSplitOptions.RemoveEmptyEntries)
                .Select(w => w.Trim())
                .Where(w => !string.IsNullOrEmpty(w))
                .Count();
            AverageCount = SentenceCount == 0 ? (int?)null : TotalCount / SentenceCount;
        }
    }
}
