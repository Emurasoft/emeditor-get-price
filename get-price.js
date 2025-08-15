/**
 * Fetches price data from the Emeditor price endpoint.
 * On any error (network error, non-2xx status, invalid JSON), returns null.
 * On success, returns the parsed JSON object from the response.
 * May throw error.
 *
 * @returns {Promise<object|null>} JSON object on success, or null on error.
 */
async function getPrice() {
  const url = 'https://emeditor-get-price.emeditor.com/';

    const res = await fetch(url, { method: 'GET' });

    // HTTP error statuses
    if (!res || !res.ok) {
        return null;
    }

    // Expect JSON; parse and return as object
    // If parsing fails, treat as error and return null
    const data = await res.json();
    // Ensure we return an object (not null/primitive)
    if (data !== null && (typeof data === 'object' || Array.isArray(data))) {
        return data;
    }
    return null;
}
