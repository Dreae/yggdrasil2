import '../styles/login-page.css';
<login-page>
  <div class="login-container">
    <div class="login-card card">
      <div class="header">
        Yggdrasil
      </div>
      <div class="login-form">
        <form onsubmit={ login }>
          <div class=form-row>
            <i class="material-icons">person</i>
            <div class="input-field">
              <input type="text" id="username">
              <label for="username">Username</label>
            </div>
          </div>
          <div class=form-row>
            <i class="material-icons">fingerprint</i>
            <div class="input-field">
              <input type="password" id="password">
              <label for="password">Password</label>
            </div>
          </div>
          <div class="form-row" style="margin-top: 48px;">
            <button class="btn grey lighten-1 waves-effect waves-light" style="width: 98px">
              Login
            </button>
          </div>
          <div class="form-row">
            <h5 class="error" if={ error }>{ error }</h5>
          </div>
        </form>
      </div>
    </div>
  </div>
  <script>
    import api from '../js/api.js';

    this.login = (e) => {
      api.login(e.target[0].value, e.target[1].value).then((info) => {
        riot.route('dashboard');
      }, (err) => {
        this.update({error: "Invalid username or password"});
      });
    }
  </script>
</login-page>
