import asyncio
import random

# FICTIONAL API ENDPOINT
async def simulated_api_call(request_id: int) -> str:
    await asyncio.sleep(random.uniform(0.1, 0.5))  # NETWORK DELAY
    if random.random() < 0.2:  # 20% FAIL CHANCE
        raise Exception(f"Request {request_id} failed")
    return f"Response for request {request_id}"

# REQUEST HANDLER
async def handle_request_limited(semaphore: asyncio.Semaphore, request_id: int) -> str:
    max_retries = 3
    retry_count = 0
    
    while retry_count < max_retries:
        async with semaphore:
            try:
                response = await simulated_api_call(request_id)
                print(f"Request {request_id} succeeded: {response}")
                return response
            except Exception as e:
                retry_count += 1
                print(f"Request {request_id} retry {retry_count}/{max_retries} failed: {e}")
                await asyncio.sleep(2 ** retry_count) 

    print(f"Request {request_id} failed after {max_retries} retries")
    return f"Failed request {request_id}"

async def handle_request(semaphore: asyncio.Semaphore, request_id: int) -> str:

    retry_count = 0
    
    while True:
        async with semaphore:
            try:
                response = await simulated_api_call(request_id)
                print(f"Request {request_id} succeeded: {response}")
                return response
            except Exception as e:
                retry_count += 1
                print(f"Request {request_id} retry({retry_count}) failed: {e}")
                await asyncio.sleep(2 ** retry_count) 

async def main():
    num_requests = 20
    semaphore = asyncio.Semaphore(5)  # MAX 5

    tasks = [handle_request(semaphore, i) for i in range(1, num_requests + 1)]
    #tasks = [handle_request_limited(semaphore, i) for i in range(1, num_requests + 1)]
    results = await asyncio.gather(*tasks)
    
    print("All requests completed.")    
    return results

if __name__ == "__main__":
    asyncio.run(main())
