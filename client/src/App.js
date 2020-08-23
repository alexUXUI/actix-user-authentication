import React from 'react';
import './App.css';

function App() {

  const login = async () => {

    let data = {
      "name": "miguel", 
      "password": "123"
    };

    let request = await fetch("http://localhost:3000/app/login", { 
        method: "POST", 
        body: JSON.stringify(data),
        headers: { 
          'Content-Type': 'application/json' ,
          'Alex': "bennett",
          'Authorization': "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE1OTg3MDk1NDF9.WE2r1AvAGSO5s0nZkIrIq67S0rQpNhPZzM5f2OqWFDk"
        },
    }).then(data => {

      console.log(data)
      const response = data.json();
      console.log('logged up!');
      console.log(response)

      return response;
    });
    
    return request;
  }

  const getUsers = async () => {
      let request = await fetch("http://localhost:3000/users/all", { 
        method: "GET", 
        headers: {
          "DAAAA": "FUUUUCK",
          'Authorization': "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE1OTg3MDk1NDF9.WE2r1AvAGSO5s0nZkIrIq67S0rQpNhPZzM5f2OqWFDk"
        }
    }).then(data => {

      console.log(data)
      const response = data.json();
      console.log('users!');
      console.log(response)

      return response;
    });
    
    return request;
  }

  return (
    <div className="App">
      <button 
        onClick={(event) => {
          login()
        }}
      >
        Login
      </button>

      <button 
        onClick={(event) => {
          getUsers()
        }}
      >
        getUsers
      </button>
    </div>
  );
}

export default App;
