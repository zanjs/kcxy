<template>
  <div class="hello">
    <h1>{{ msg }}</h1>
    <label>Email ：&emsp;&emsp;<input type="text" name="email" v-model="Email"></label> <br>
    <label>Username ：<input type="text" name="username" v-model="Username"></label> <br>
    <label>Password ：<input type="text" name="password" v-model="Password"></label> <br>
    <p>
      <button @click="addUser">Add</button>&emsp;
      <a href="javascript:;" @click="getUser">查询</a>
    </p>
  </div>
</template>

<script>
export default {
  name: 'hello',
  data () {
    return {
      msg: 'There is something cool',
      Email: '',
      Username: '',
      Password: ''
    }
  },
  methods: {
    addUser () {
      var email = this.Email
      var username = this.Username
      var password = this.Password

      // this.$http.post('http://localhost:8000/api/user/addUser',{
      this.$http.post('/api/user/register', {
          email: email,
          username: username,
          password: password
      })
      .then((response) => {
        console.log(response)
      })
      .catch((error) => {
        console.log(error)
      })
    },
    getUser(){
        // this.$http.get('http://localhost:8000/api/user/getUser',{
        this.$http.get('/api/user/getUser',{
            params: {
                  id: 1
            }
            }).then((response)=>{
                  console.log(response);
            })
        }
    }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h1, h2 {
  font-weight: normal;
}

ul {
  list-style-type: none;
  padding: 0;
}

li {
  display: inline-block;
  margin: 0 10px;
}

a {
  color: #42b983;
}
p {
  width: 50%;
  margin: 0 auto;
  margin-top: 20px;
}
p label {
  width: 80px;
  display: inline-block
}
</style>
