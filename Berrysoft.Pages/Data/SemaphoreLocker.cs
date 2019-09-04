﻿using System;
using System.Threading;
using System.Threading.Tasks;

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
}
