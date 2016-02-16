import '../styles/login-page.css';
<login-page>
  <div class="container">
    <div class="login-card mdl-card mdl-shadow--2dp">
      <div class="mld-card__title header">
        Yggdrasil
      </div>
      <div class="login-form">
        <form onsubmit={ login }>
          <div class=form-row>
            <i class="material-icons">person</i>
            <div class="mdl-textfield mdl-js-textfield mdl-needs-upgrade">
              <input class="mdl-textfield__input" type="text" id="username">
              <label class="mdl-textfield__label" for="username">Username</label>
            </div>
          </div>
          <div class=form-row>
            <i class="material-icons">fingerprint</i>
            <div class="mdl-textfield mdl-js-textfield mdl-needs-upgrade">
              <input class="mdl-textfield__input" type="password" id="password">
              <label class="mdl-textfield__label" for="password">Password</label>
            </div>
          </div>
          <div class="form-row" style="margin-top: 64px;">
            <button class="mdl-button mdl-js-button mdl-button--raised mdl-js-ripple-effect" style="width: 98px">
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

    this.on('mount', () => {
      componentHandler.upgradeElements(document.getElementsByClassName("mdl-needs-upgrade"));
    });

    this.login = (e) => {
      api.login(e.target[0].value, e.target[1].value).then((info) => {
        riot.route('dashboard');
      }, (err) => {
        this.update({error: "Invalid username or password"});
      });
    }
  </script>
</login-page>
