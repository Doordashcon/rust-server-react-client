import axios from 'axios';
import { useState } from 'react';
import { INVITATION } from './config';
import { REGISTER } from './config';
import { FOMO } from './config';

const App = () => {

	const [path, setPath] = useState();

	const invitation = () => {
		axios.post(
			`${INVITATION}`, 
			{
				email: "doordashcon@gmaill.com"
			},
			{ 
				withCredentials: true 
			}
		).then(res => console.log(res))
	}

	const register = () => {
		axios.post(
			`${REGISTER}/${path}`, 
			{
				password: "pikachu"
			},
			{ 
				withCredentials: true 
			}
		).then(res => console.log(res))
	}

	const fomo = () => {
		axios.post(
			`${FOMO}`, 
			{
				email: "doordashcon@gmaill.com",
				password: "pikachu"
			},
			{
				withCredentials: true
			}
		).then(res => console.log(res))
	}

	return(
		<>
			<div>
				<button onClick={invitation}>
				Invitation
				</button><br />
				<form>
					<input type="text" onChange={(e) => setPath(e.target.value)}/>
					<button onClick={register}>
					register
					</button>
				</form>
				<br />
				<button onClick={fomo}>
				fomo
				</button>
			</div>
		</>
	)
}

export default App;
