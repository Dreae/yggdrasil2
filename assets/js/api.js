import request from 'superagent';

function get_session() {
  return new Promise((resolve, reject) => {
    request.get("/api/restricted/session").end((err, res) => {
      if(err) {
        reject(err);
      } else {
        resolve(JSON.parse(res.text));
      }
    });
  });
}

function login(username, password) {
  return new Promise((resolve, reject) => {
    request.post("/api/login")
      .send({username: username, password: password})
      .end((err, res) => {
        if(err) {
          reject(err);
        } else {
          resolve(JSON.parse(res.text));
        }
      });
  });
}

module.exports = {
  get_session: get_session,
  login: login
};
