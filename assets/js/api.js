import request from 'superagent';

function get_session() {
  return new Promise((resolve, reject) => {
    request.get("/api/session").end((err, res) => {
      if(err) {
        reject(err);
      } else {
        resolve(JSON.parse(res.text));
      }
    });
  });
}

module.exports = {
  get_session: get_session
};
