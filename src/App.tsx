import { HashRouter as Router, Routes, Route } from 'react-router-dom';
import Home from './Home';
import NewPage from './NewPage'; // Add the new page

const App = () => {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/new-page" element={<NewPage />} /> {/* Add this route */}
      </Routes>
    </Router>
  );
};

export default App;
