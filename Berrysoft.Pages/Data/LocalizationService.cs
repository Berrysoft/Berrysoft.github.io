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
    // https://stackoverflow.com/questions/7612602/why-cant-i-use-the-await-operator-within-the-body-of-a-lock-statement
    class SemaphoreLocker
    {
        private readonly SemaphoreSlim _semaphore = new SemaphoreSlim(1, 1);

        public async Task LockAsync(Func<Task> worker)
        {
            await _semaphore.WaitAsync();
            try
            {
                await worker();
            }
            finally
            {
                _semaphore.Release();
            }
        }

        public async Task<T> LockAsync<T>(Func<Task<T>> worker)
        {
            await _semaphore.WaitAsync();
            try
            {
                return await worker();
            }
            finally
            {
                _semaphore.Release();
            }
        }
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

        private string GetCompatibleLanguage(string lang)
        {
            while (lang != null && !languages.ContainsKey(lang))
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
            var document = await GetStringsAsync(Language ?? InvarientLanguage);
            if (document != null && document.TryGetValue(key, out var prop))
            {
                return prop;
            }
            else
            {
                return null;
            }
        }
    }
}
