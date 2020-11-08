import React, {useState} from 'react';
const axios = require("axios");

const initial = {
  name: '',
  price: '',
  quantity: '',
}

function App() {
  const [form, setForm] = useState(initial);
  const {name, price, quantity} = form;

  const onSubmit = async (e) => {
    e.preventDefault();
    const {data} = await axios.post("/api/v1/index", form, {
      headers: {
        'content-type': 'application/json'
      }
    });
    console.log(data.json());
    setForm(initial);
  }
  const onChange = (e) => setForm({ ...form, [e.target.name]: e.target.value })

  return (
    <React.Fragment>
    <div className="container" style={{margin: '5rem'}}>
        <form className="needs-validation" noValidate>
          <div className="form-row">
            <div className="col-md-6 mb-3">
              <label>Name</label>
              <input type="text" className="form-control" name="name" value={name} onChange={(e) => onChange(e)} required/>
            </div>
            <div className="col-md-6 mb-3">
              <label>Price</label>
              <input type="text" className="form-control" name="price" value={price} onChange={(e) => onChange(e)} required/>
            </div>
          </div>
          <div className="form-row">
            <div className="col-md-3 mb-3">
              <label>Quantity</label>
              <input type="text" className="form-control" name="quantity" value={quantity} onChange={(e) => onChange(e)} required/>
            </div>
          </div>
          <button className="btn btn-primary" onSubmit={(e) => onSubmit(e)}>Submit form</button>
        </form>
      </div>
    </React.Fragment>
  );
}

export default App;
