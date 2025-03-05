type SearchCondition = {
    card_page: number,
    keyword: string,
    product_type: 'booster' | 'starter',
    product_id: string,
    product_no: string,
    card_kind: string,
    card_type: string,
    rarelity: string,
    support_formats: string,
    story: string,
    level: string,
    color: string,
    ability: string,
};

type SendRequestAndCacheOption<T> = {
    method: string,
    endpoint: string,
    payload: T,
    selector_to_pick: string,
    referrer: string,
    url_separator: string,
    text_cache_dir: string,
    force_update: boolean,
    virtual_product_no?: string
}

export {
    type SearchCondition,
    type SendRequestAndCacheOption
};