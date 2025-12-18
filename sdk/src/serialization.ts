/**
 * Deterministic JSON serialization for hashing.
 */
export const serialize = (data: any): Buffer => {
    // Sort keys recursively
    const sortKeys = (obj: any): any => {
        if (typeof obj !== 'object' || obj === null) return obj;
        if (Array.isArray(obj)) return obj.map(sortKeys);
        return Object.keys(obj).sort().reduce((acc: any, key) => {
            acc[key] = sortKeys(obj[key]);
            return acc;
        }, {});
    };
    return Buffer.from(JSON.stringify(sortKeys(data)));
};