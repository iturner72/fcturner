import React, { useEffect, useState } from 'react';
import './App.css';
import PollOption from './components/PollOption';

const App: React.FC = () => {
    const [options, setOptions] = useState<string[]>([]);

    useEffect(() => {
        fetch('http://localhost:8080/poll')
            .then(response => response.json())
            .then(data => {
                console.log("Fetched data:", data);
                setOptions(data);
            })
            .catch(error => console.error('Error fetching poll options:', error));
    }, []);

    const handleSubmit = (option: string) => {
        console.log(option + ' was clicked');

        fetch('http://localhost:8080/poll', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ option: option }),
        })
        .then(response => response.json())
        .then(data => {
            console.log('Vote successful:', data);
        })
        .catch((error) => {
            console.error('Error:', error);
        });
    };

    return (
        <div className="bg-gray-900 min-h-screen text-white flex justify-center items-center">
            <div>
                {Array.isArray(options) && options.map(option => (
                    <PollOption key={option} option={option} onSubmit={handleSubmit} />
                ))}
            </div>
        </div>
    );
};

export default App;
