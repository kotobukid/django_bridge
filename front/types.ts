export type Product = {
    product_code: string,
    name: string,
    id: number,
    url?: string,
    product_type: 'bo' | 'st' | 'pr',
    sort_asc: number
}