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
}
