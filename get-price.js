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
    const el = document.getElementById(id);
    if (!el) {
        console.error(`get-price: element with id ${id} not found`);
        return;
    }

    if (typeof priceStr === 'string') {
        el.textContent = priceStr;
    }
}

// Output prices to elements
document.addEventListener('DOMContentLoaded', async () => {
    let price;
    try {
        price = await getPrice();
    } catch (_e) {
        return
    }

    const idToKey = {
        'buynow-annual-price': 'annual',
        'buynow-annual-per-month-price': 'annual_per_month',
        'buynow-annual-renewal-price': 'annual_renewal_price',
        'buynow-monthly-price': 'monthly',
        'buynow-monthly-renewal-price': 'monthly'
    };

    for (const id of Object.keys(idToKey)) {
        await outputPrice(id, price[idToKey[id]]);
    }
});