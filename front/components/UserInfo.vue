<script setup lang="ts">
import {computed, onMounted, ref} from "vue";
import {useAuthStore} from "../stores/auth";

const auth_store = useAuthStore();

const first_loaded = ref<boolean>(false);

const login_id = ref<string>('');
const password = ref<string>('');

const mode = ref<"signup" | "login">("login");

const submit = () => {
  auth_store.login({login_id: login_id.value, password: password.value}).then((_login_id: string) => {
    login_id.value = _login_id;
  }).catch(reason => {
    alert(reason);
  });
};

const dispatch_logout = () => {
  auth_store.logout().then(() => {
    login_id.value = '';
    password.value = '';
  });
};

const products_page = computed(() => {
  return location.port === '3001' ? '/products.html' : '/products';
});

onMounted(() => {
  auth_store.fetch_user_info().then(() => {
    first_loaded.value = true;
  });
});

const name_new = ref<string>('');
const login_id_new = ref<string>('');
const password_new = ref<string>('');
const password_confirm = ref<string>('');
const prevent_double_submit = ref<boolean>(false);
const register = (): void => {
  if (prevent_double_submit.value) {
    return;
  }
  prevent_double_submit.value = true;
  auth_store.create_user({
    name: name_new.value,
    login_id: login_id_new.value,
    password: password_new.value,
    password_confirm: password_confirm.value,
  }).then((): void => {
    prevent_double_submit.value = false
  });
};

</script>

<template lang="pug">
  .bar(v-if="!first_loaded")
    span &nbsp;
  .bar.nav.authenticated(v-if="auth_store.login_id && first_loaded")
    span {{ auth_store.name }}
    a(href="#" @click.prevent="dispatch_logout") ログアウト
    a(:href="products_page" target="_blank" v-if="auth_store.is_admin") 製品管理
  .bar.not_authenticated(v-if="!auth_store.login_id && first_loaded")
    form.login(action="/api/login" method="POST" @submit.prevent="submit" v-if="mode === 'login'")
      label
        span ID:
        input(type="text" v-model.lazy="login_id")
      label
        span Password:
        input(type="password" v-model.lazy="password")
      input(type="submit" value="Login")
    a.toggle_mode(href="#" @click="mode='signup'" v-if="mode==='login'") Signup &gt;&gt;
    a.toggle_mode(href="#" @click="mode='login'" v-if="mode==='signup'") &lt;&lt; Login
    form.signup(v-if="mode === 'signup'")
      .popup
        .inner
          label
            span ユーザー名
            input(type="text" v-model="name_new" autocomplete="off")
          br
          label
            span 希望ログインID
            input(type="text" v-model="login_id_new" autocomplete="off")
          br
          label
            span パスワード
            input(type="password" v-model="password_new" autocomplete="off")
          br
          label
            span パスワード(確認)
            input(type="password" v-model="password_confirm" autocomplete="off")
          br
          label
            span &nbsp;
            button(@click.prevent="register") 登録
</template>

<style scoped lang="less">
@import "../composable/button.less";

.popup {
  padding: 50px;
  width: calc(100vw - 120px);
  height: calc(100vh - 180px);
  position: absolute;
  top: 60px;
  left: 10px;
  background-color: #505050;
  border-color: black;
  border-radius: 10px;
}

.inner {
  opacity: 1;
}

form.login {
  width: 600px;
  display: inline-block;
}

form.signup {
  width: 400px;
  display: inline-block;
  color: white;

  label {
    margin-bottom: 10px;
  }

  label span {
    display: inline-block;
    width: 120px;
  }
}

a.toggle_mode {
  width: 100px;
  color: white;
  display: inline-block;

  &:hover {
    text-decoration: underline;
  }
}

.bar {
  background-color: #313131;

  &.authenticated {
    background-color: #313131;
  }

  &.not_authenticated {
    background-color: #797979;
    color: black;
  }

  padding: 10px;
  color: white;

  label {
    span {
      display: inline-block;
      margin-right: 5px;
    }

    display: inline-block;
    margin-right: 2rem;
  }
}

.nav {
  a {
    .small_button();
  }
}
</style>