using System;
using System.Threading.Tasks;

namespace Berrysoft.Pages.Data
{
    public interface ILocalizationService
    {
        string Language { get; set; }
        int LanguageIndex { get; set; }
        Task<string> GetStringAsync(string key);
        event EventHandler<string> LanguageChanged;
    }
}
