using System;
using System.Collections.Generic;
using System.Globalization;
using System.Net.Http;
using System.Text.Json;
using System.Threading;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Components;

namespace Berrysoft.Pages.Data
{
    public interface ILocalizationService
    {
        string Language { get; set; }
        Task<string> GetStringAsync(string key);
        Task<Dictionary<string, string>> GetLanguagesAsync();
        event EventHandler<string> LanguageChanged;
    }

    public class LocalizationService : ILocalizationService
    {
        protected HttpClient Http { get; set; }

        public LocalizationService(HttpClient http)
        {
            Http = http;
            strings = new Dictionary<string, Dictionary<string, string>>();
            Language = Thread.CurrentThread.CurrentUICulture.Name;
        }

        private const string InvarientLanguage = "invarient";

        private Dictionary<string, string> languages;
        private static readonly SemaphoreLocker languagesLocker = new SemaphoreLocker();
        private Dictionary<string, Dictionary<string, string>> strings;
        private static readonly SemaphoreLocker stringsLocker = new SemaphoreLocker();

        private string language;
        public string Language
        {
            get => language;
            set => SetLanguage(value);
        }
        private async void SetLanguage(string value)
        {
            await InitializeLanguages();
            value = GetCompatibleLanguage(value);
            if (language != value)
            {
                language = value;
                LanguageChanged?.Invoke(this, Language);
            }
        }

        public event EventHandler<string> LanguageChanged;

        private Task InitializeLanguages()
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
                return Task.CompletedTask;
            }
        }

        public async Task<Dictionary<string, string>> GetLanguagesAsync()
        {
            await InitializeLanguages();
            return languages;
        }

        private string GetParentLanguage(string lang)
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

        private string GetCompatibleLanguage(string lang)
        {
            while (lang != null && !languages.ContainsKey(lang))
            {
                if (string.IsNullOrEmpty(lang))
                    return InvarientLanguage;
                lang = GetParentLanguage(lang);
            }
            return lang ?? InvarientLanguage;
        }

        private async Task<(string, string)> GetStringsFileNameAsync(string lang)
        {
            await InitializeLanguages();
            if (string.IsNullOrEmpty(lang) || lang == InvarientLanguage)
            {
                return (InvarientLanguage, "i18n/strings.json");
            }
            else
            {
                return (lang, $"i18n/strings.{lang}.json");
            }
        }

        private Task<Dictionary<string, string>> GetStringsAsync(string lang)
        {
            if (strings.ContainsKey(lang))
            {
                return Task.FromResult(strings[lang]);
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

        public async Task<string> GetStringAsync(string key)
        {
            string lang = Language ?? InvarientLanguage;
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
