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
 * Sets the text of a DOM element (by id) to the requested price field.
 *
 * @param {string} id - The id of the target DOM element.
 * @param {'annual'|'monthly'} plan - Which price field to display.
 * @returns {Promise<void>}
 */
async function outputPrice(id, plan) {
    if (!id || (plan !== 'annual' && plan !== 'monthly')) {
        return;
    }
    const el = document.getElementById(id);
    if (!el) {
        return;
    }

    try {
        const price = await getPrice();
        if (!price || typeof price !== 'object') {
            return;
        }
    } catch (_e) {
        return;
    }

    const value = price[plan];
    if (typeof value === 'string' || typeof value === 'number') {
        el.textContent = String(value);
    }
}

