using System;
using System.Collections.Generic;
using System.Threading.Tasks;

namespace Berrysoft.Pages.Data
{
    public interface ILocalizationService
    {
        string Language { get; set; }
        Task<string> GetStringAsync(string key);
        Task<Dictionary<string, string>> GetLanguagesAsync();
        event EventHandler<string> LanguageChanged;
    }
}
