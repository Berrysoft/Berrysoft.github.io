﻿using System.Collections.Generic;
using System.Net.Http;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Components;

namespace Berrysoft.Pages.Data
{
    public interface IDataLoaderService<T>
    {
        ValueTask LoadDataAsync();
        T Data { get; }
    }

    public class DataLoaderService<T> : IDataLoaderService<T?>
        where T : class
    {
        protected string Uri { get; set; }
        protected HttpClient Http { get; set; }

        public DataLoaderService(string uri, HttpClient http)
        {
            Uri = uri;
            Http = http;
        }

        public T? Data { get; private set; }
        private static readonly SemaphoreLocker dataLocker = new SemaphoreLocker();

        public ValueTask LoadDataAsync()
        {
            if (Data == null)
            {
                return dataLocker.LockAsync(async () =>
                {
                    if (Data == null)
                    {
                        Data = await Http.GetJsonAsync<T>(Uri);
                    }
                });
            }
            else
            {
                return new ValueTask();
            }
        }
    }

    public class EnumerableLoaderService<T> : DataLoaderService<IEnumerable<T>>
    {
        public EnumerableLoaderService(string uri, HttpClient http) : base(uri, http) { }
    }
}
