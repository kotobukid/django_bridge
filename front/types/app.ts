type Env = {
    // app
    IMAGE_CACHE_DIR?: string,
    TEXT_CACHE_DIR?: string,

    // prisma
    DATABASE_URL: string
}

type User = {
    login_id: string,
    id: number,
    name: string,
    password: string,
    theme: string,
    last_login: Date,
    created_at: Date,
    is_admin: boolean,
    use_allstar: boolean,
    use_key_selection: boolean
}

type LoginInfo = {
    login_id: string,
    password: string
}

type Product = {
    id: number,
    name: string,
    product_no: string,
    product_type: string,
    last_fetched?: Date,
    last_converted?: Date,
    sort: number,
    processing: boolean
}

export type {
    Env,
    User,
    Product,
    LoginInfo
}
