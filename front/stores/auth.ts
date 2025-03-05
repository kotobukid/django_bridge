import {defineStore} from "pinia";
import axios, {type AxiosResponse} from "axios";
import {type User} from "../types/app"

type State = {
    is_admin: boolean,
    login_id: string,
    name: string
};

const useAuthStore = defineStore('auth', {
    state(): State {
        return {
            is_admin: false,
            login_id: '',
            name: ''
        }
    },
    actions: {
        login({login_id, password}: { login_id: string, password: string }): Promise<string> {
            return new Promise<string>((resolve, reject): void => {
                if (login_id && password) {
                    axios.post('/api/auth/login', {
                        login_id,
                        password
                    }).then((res: AxiosResponse<{ success: boolean, reason?: string, user: User }>): void => {
                        if (res.data.success && res.data.user.login_id) {
                            this.name = res.data.user.name;
                            this.login_id = res.data.user.login_id;
                            this.is_admin = res.data.user.is_admin;

                            resolve(res.data.user.login_id);
                        } else {
                            reject(new Error('ログインに失敗しました。ログインIDまたはパスワードが不正です。'));
                        }
                    });
                } else {
                    reject(new Error('ログインIDとパスワードを入力してください'));
                }
            });
        },
        logout(): Promise<void> {
            return new Promise(resolve => {
                axios.post('/api/auth/logout').then((): void => {
                    this.name = '';
                    this.login_id = '';
                    this.is_admin = false;
                    resolve();
                });
            })
        },
        fetch_user_info(): Promise<void> {
            return new Promise<void>(resolve => {
                axios.get('/api/auth/').then((res: AxiosResponse<{
                    login_id: string,
                    name: string,
                    is_admin: boolean
                }>): void => {
                    this.name = res.data.name;
                    this.login_id = res.data.login_id;
                    this.is_admin = res.data.is_admin;
                    resolve();
                });
            });
        },
        create_user({name, login_id, password, password_confirm}: {
            name: string,
            login_id: string,
            password: string,
            password_confirm: string
        }): Promise<void> {
            return new Promise((resolve): void => {
                axios.post('/api/auth/create_user', {
                    name,
                    login_id,
                    password,
                    password_confirm
                }).then((res: AxiosResponse<{
                    success: boolean,
                    reason?: string,
                    user: User
                }>): void => {
                    alert('登録しました。');
                    this.name = res.data.user.name;
                    this.login_id = res.data.user.login_id;
                    this.is_admin = res.data.user.is_admin;
                    resolve();
                }).catch((error: {response: {data: {reason: string}}}): void => {
                    alert(error.response.data.reason!);
                    resolve();
                });
            });
        }
    }
});

export {useAuthStore};