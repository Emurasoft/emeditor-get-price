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

    const res = await fetch(url, {method: 'GET'});

    // HTTP error statuses
    if (!res || !res.ok) {
        return null;
    }

    // Expect JSON; parse and return as object
    // If parsing fails, treat as error and return null
    const data = await res.json();
    // Ensure we return an object (not null/primitive)
    if (data !== null && typeof data === 'object') {
        return data;
    }
    return null;
}

/**
 * Sets the text of a DOM element (by id) to the provided price string.
 *
 * Instead of fetching inside, this function only outputs the given string.
 * Call getPrice() before invoking this function.
 *
 * @param {string} id - The id of the target DOM element.
 * @param {string} priceStr - The price string (or number) to display.
 * @returns {Promise<void>}
 */
async function outputPrice(id, priceStr) {
    if (!id) {
        return;
    }
    const el = document.getElementById(id);
    if (!el) {
        return;
    }

    if (typeof priceStr === 'string') {
        el.textContent = priceStr;
    }
}

// Auto-initialize after DOM is ready
document.addEventListener('DOMContentLoaded', async () => {
    let price;
    try {
        price = await getPrice();
    } catch (_e) {
        return
    }

    await outputPrice('buynow-annual-price', price['annual']);
    await outputPrice('buynow-annual-per-month-price', price['annual_per_month']);
});