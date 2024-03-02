import React from 'react';

interface PollOptionProps {
    option: string;
    onSubmit: (option: string) => void;
}

const PollOption: React.FC<PollOptionProps> = ({ option, onSubmit }) => {
    return (
        <button onClick={() => onSubmit(option)} className="bg-purple-800 hover:bg-purple-700 text-white font-bold py-2 px-4 rounded m-2">{option}</button>
    );
};

export default PollOption;
