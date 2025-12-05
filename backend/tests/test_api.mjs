const csv = `ISWC,Title,BPM,Key,Creator IPI,Creator Role,Instrumental
T1234567890,Test Song Alpha,120,C Major,111111111,Composer,false
T1234567890,Test Song Alpha,120,C Major,222222222,Author,false`;

const formData = new FormData();
formData.append('file', new Blob([csv], { type: 'text/csv' }), 'test.csv');

const res = await fetch('http://localhost:3000/api/upload', { method: 'POST', body: formData });
const data = await res.json();

console.log('Keys:', Object.keys(data));
console.log('Status:', data.status);
console.log('Grouped:', data.grouped?.length || data.musicalWorks?.length || 'none');
console.log('Sample:', JSON.stringify(data.grouped?.[0] || data.musicalWorks?.[0], null, 2)?.substring(0, 500));
