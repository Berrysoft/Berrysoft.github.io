using System.Collections.Generic;
using System.Globalization;
using System.Net.Http;
using System.Text.Json;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Components;

namespace Berrysoft.Pages.Data
{
    public delegate void LanguageChangedCallback(object sender, string lang);
    public delegate ValueTask LanguageChangedAsyncCallback(object sender, string lang);

    public interface ILocalizationService : IDataLoaderService<IReadOnlyDictionary<string, string>?>
    {
        string? Language { get; set; }
        CultureInfo Culture { get; }
        ValueTask SetLanguageAsync(string? value);
        ValueTask<string?> GetStringAsync(string key);
        event LanguageChangedCallback? LanguageChanged;
        event LanguageChangedAsyncCallback? LanguageChangedAsync;
    }

    public class LocalizationService : ILocalizationService
    {
        protected HttpClient Http { get; set; }

        public LocalizationService(HttpClient http)
        {
            Http = http;
            strings = new Dictionary<string, Dictionary<string, string>>();
        }

        private const string InvarientLanguage = "invarient";

        private Dictionary<string, string>? languages;
        public IReadOnlyDictionary<string, string>? Data => languages;
        private static readonly SemaphoreLocker languagesLocker = new SemaphoreLocker();
        private Dictionary<string, Dictionary<string, string>> strings;
        private static readonly SemaphoreLocker stringsLocker = new SemaphoreLocker();

        private string? language;
        public string? Language
        {
            get => language;
            set => SetLanguage(value);
        }
        private async void SetLanguage(string? value) => await SetLanguageAsync(value);

        public async ValueTask SetLanguageAsync(string? value)
        {
            await LoadDataAsync();
            value = GetCompatibleLanguage(value);
            if (language != value)
            {
                language = value;
                await OnLanguageChangedAsync(language);
            }
        }

        public CultureInfo Culture => CultureInfo.GetCultureInfo((Language == null || Language == InvarientLanguage) ? string.Empty : Language);

        public event LanguageChangedCallback? LanguageChanged;
        public event LanguageChangedAsyncCallback? LanguageChangedAsync;

        protected virtual async ValueTask OnLanguageChangedAsync(string lang)
        {
            if (LanguageChangedAsync != null)
            {
                await LanguageChangedAsync(this, lang);
            }
            LanguageChanged?.Invoke(this, lang);
        }

        public ValueTask LoadDataAsync()
        {
            if (languages == null)
            {
                return languagesLocker.LockAsync(async () =>
                {
                    if (languages == null)
                    {
                        languages = await Http.GetJsonAsync<Dictionary<string, string>>("i18n/index.json");
                    }
                });
            }
            else
            {
                return new ValueTask();
            }
        }

        private string? GetParentLanguage(string? lang)
        {
            try
            {
                var culture = CultureInfo.GetCultureInfo(lang);
                lang = culture.Parent.Name.ToLower();
            }
            catch (CultureNotFoundException)
            {
                lang = null;
            }
            return lang;
        }

        private string GetCompatibleLanguage(string? lang)
        {
            while (lang != null && !languages!.ContainsKey(lang))
            {
                if (string.IsNullOrEmpty(lang))
                    return InvarientLanguage;
                lang = GetParentLanguage(lang);
            }
            return lang ?? InvarientLanguage;
        }

        private async ValueTask<(string, string)> GetStringsFileNameAsync(string lang)
        {
            await LoadDataAsync();
            if (string.IsNullOrEmpty(lang) || lang == InvarientLanguage)
            {
                return (InvarientLanguage, "i18n/strings.json");
            }
            else
            {
                return (lang, $"i18n/strings.{lang}.json");
            }
        }

        private ValueTask<Dictionary<string, string>> GetStringsAsync(string lang)
        {
            if (strings.ContainsKey(lang))
            {
                return new ValueTask<Dictionary<string, string>>(strings[lang]);
            }
            else
            {
                return stringsLocker.LockAsync(async () =>
                {
                    if (strings.ContainsKey(lang))
                    {
                        return strings[lang];
                    }
                    else
                    {
                        var (realLang, filename) = await GetStringsFileNameAsync(lang);
                        var document = JsonSerializer.Deserialize<Dictionary<string, string>>(await Http.GetByteArrayAsync(filename));
                        strings[realLang] = document;
                        return document;
                    }
                });
            }
        }

        public async ValueTask<string?> GetStringAsync(string key)
        {
            string? lang = Language ?? InvarientLanguage;
            while (lang != null)
            {
                var document = await GetStringsAsync(lang);
                if (document != null && document.TryGetValue(key, out var prop))
                {
                    return prop;
                }
                if (lang == InvarientLanguage)
                {
                    lang = null;
                }
                else
                {
                    lang = GetCompatibleLanguage(GetParentLanguage(lang));
                }
            }
            return null;
        }
    }
}
